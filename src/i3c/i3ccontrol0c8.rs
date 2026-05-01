#[doc = "Register `I3CCONTROL0C8` reader"]
pub type R = crate::R<I3ccontrol0c8Spec>;
#[doc = "Register `I3CCONTROL0C8` writer"]
pub type W = crate::W<I3ccontrol0c8Spec>;
#[doc = "Field `REGGETMXDSMAXRDTURNL` reader - REG_GETMXDS_MAXRDTURN_L"]
pub type ReggetmxdsmaxrdturnlR = crate::FieldReader;
#[doc = "Field `REGGETMXDSMAXRDTURNL` writer - REG_GETMXDS_MAXRDTURN_L"]
pub type ReggetmxdsmaxrdturnlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGGETMXDSMAXRDTURNM` reader - REG_GETMXDS_MAXRDTURN_M"]
pub type ReggetmxdsmaxrdturnmR = crate::FieldReader;
#[doc = "Field `REGGETMXDSMAXRDTURNM` writer - REG_GETMXDS_MAXRDTURN_M"]
pub type ReggetmxdsmaxrdturnmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGGETMXDSMAXRDTURNH` reader - REG_GETMXDS_MAXRDTURN_H"]
pub type ReggetmxdsmaxrdturnhR = crate::FieldReader;
#[doc = "Field `REGGETMXDSMAXRDTURNH` writer - REG_GETMXDS_MAXRDTURN_H"]
pub type ReggetmxdsmaxrdturnhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGGETMXDSEN` reader - REG_GETMXDS_EN"]
pub type ReggetmxdsenR = crate::BitReader;
#[doc = "Field `REGGETMXDSEN` writer - REG_GETMXDS_EN"]
pub type ReggetmxdsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPENDINGINTHW` reader - REG_PENDING_INT_HW"]
pub type RegpendinginthwR = crate::BitReader;
#[doc = "Field `REGPENDINGINTHW` writer - REG_PENDING_INT_HW"]
pub type RegpendinginthwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_GETMXDS_MAXRDTURN_L"]
    #[inline(always)]
    pub fn reggetmxdsmaxrdturnl(&self) -> ReggetmxdsmaxrdturnlR {
        ReggetmxdsmaxrdturnlR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_GETMXDS_MAXRDTURN_M"]
    #[inline(always)]
    pub fn reggetmxdsmaxrdturnm(&self) -> ReggetmxdsmaxrdturnmR {
        ReggetmxdsmaxrdturnmR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_GETMXDS_MAXRDTURN_H"]
    #[inline(always)]
    pub fn reggetmxdsmaxrdturnh(&self) -> ReggetmxdsmaxrdturnhR {
        ReggetmxdsmaxrdturnhR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - REG_GETMXDS_EN"]
    #[inline(always)]
    pub fn reggetmxdsen(&self) -> ReggetmxdsenR {
        ReggetmxdsenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - REG_PENDING_INT_HW"]
    #[inline(always)]
    pub fn regpendinginthw(&self) -> RegpendinginthwR {
        RegpendinginthwR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_GETMXDS_MAXRDTURN_L"]
    #[inline(always)]
    pub fn reggetmxdsmaxrdturnl(&mut self) -> ReggetmxdsmaxrdturnlW<I3ccontrol0c8Spec> {
        ReggetmxdsmaxrdturnlW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_GETMXDS_MAXRDTURN_M"]
    #[inline(always)]
    pub fn reggetmxdsmaxrdturnm(&mut self) -> ReggetmxdsmaxrdturnmW<I3ccontrol0c8Spec> {
        ReggetmxdsmaxrdturnmW::new(self, 8)
    }
    #[doc = "Bits 16:23 - REG_GETMXDS_MAXRDTURN_H"]
    #[inline(always)]
    pub fn reggetmxdsmaxrdturnh(&mut self) -> ReggetmxdsmaxrdturnhW<I3ccontrol0c8Spec> {
        ReggetmxdsmaxrdturnhW::new(self, 16)
    }
    #[doc = "Bit 24 - REG_GETMXDS_EN"]
    #[inline(always)]
    pub fn reggetmxdsen(&mut self) -> ReggetmxdsenW<I3ccontrol0c8Spec> {
        ReggetmxdsenW::new(self, 24)
    }
    #[doc = "Bit 25 - REG_PENDING_INT_HW"]
    #[inline(always)]
    pub fn regpendinginthw(&mut self) -> RegpendinginthwW<I3ccontrol0c8Spec> {
        RegpendinginthwW::new(self, 25)
    }
}
#[doc = "I3C\\_SLV\\_CTL\\_0C8\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0c8Spec;
impl crate::RegisterSpec for I3ccontrol0c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0c8::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0c8Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0c8::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0C8 to value 0"]
impl crate::Resettable for I3ccontrol0c8Spec {}
