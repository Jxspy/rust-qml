#ifndef QRSDYNAMICOBJECT_H
#define QRSDYNAMICOBJECT_H

#include <QtCore>

#if Q_MOC_OUTPUT_REVISION != 67
#error "Unsupport Qt version. Qt with Q_MOC_OUTPUT_REVISION == 67 is required."
#endif

extern "C" typedef void *(QrsSlotFunction)(void *data, int slot, QVariant **args);

class QrsDynamicMetaObject{
  public:
    QrsDynamicMetaObject();
    virtual ~QrsDynamicMetaObject();
    //方法名及参数映射
    struct Method {
      QString name;
      uint args;
      uint flags;
    };
    //添加Slot
    void addSlot(QString name, uint args) {
      if (_mo) qFatal("Cannot add slot after object created");

      Method m = {name,args,0x0a};
      _methods.append(m);
    }
    //添加Signal
    void addSignal(QString name, uint args) {
      if (_mo) qFatal("Cannot add signal after object created");
      Method m = {name,args,0x06};
      
      _methods.append(m);
    }
    //创建基本qobject
    QObject *create(QrsSlotFunction fun, void *data);

  private:
    //方法名及参数映射列表
    QList<Method> _methods;
    //核心类
    QMetaObject *_mo;
    
    void finalize();
};

class QrsDynamicObject : public QObject{
  public:
    //通过一个公用的Slot去初始化对象
    QrsDynamicObject(QrsSlotFunction *fun, void *data, QMetaObject *mo, int n_slots);
    
    virtual const QMetaObject* metaObject() const;
    virtual void* qt_metacast(const char* );
    virtual int qt_metacall(QMetaObject::Call , int , void** );
    //触发信号方法
    void emitSignal(int id);

  private:
    QrsSlotFunction *_fun;
    void *_data;
    int _n_slots;

    //核心元对象，动态槽/信号
    QMetaObject *_mo;
    
    void invokeMetacall(int id, void** args);
};

#endif // QRSDYNAMICOBJECT_H
