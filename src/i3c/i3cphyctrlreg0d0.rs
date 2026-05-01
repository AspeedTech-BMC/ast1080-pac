#[doc = "Register `I3CPHYCTRLREG0D0` reader"]
pub type R = crate::R<I3cphyctrlreg0d0Spec>;
#[doc = "Register `I3CPHYCTRLREG0D0` writer"]
pub type W = crate::W<I3cphyctrlreg0d0Spec>;
#[doc = "Field `REGIDLESDAPULLUPEN` reader - REG_IDLE_SDA_PULLUP_EN"]
pub type RegidlesdapullupenR = crate::FieldReader;
#[doc = "Field `REGIDLESDAPULLUPEN` writer - REG_IDLE_SDA_PULLUP_EN"]
pub type RegidlesdapullupenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGIDLESCLPULLUPEN` reader - REG_IDLE_SCL_PULLUP_EN"]
pub type RegidlesclpullupenR = crate::FieldReader;
#[doc = "Field `REGIDLESCLPULLUPEN` writer - REG_IDLE_SCL_PULLUP_EN"]
pub type RegidlesclpullupenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGKPSDAPULLUPEN` reader - REG_KP_SDA_PULLUP_EN"]
pub type RegkpsdapullupenR = crate::FieldReader;
#[doc = "Field `REGKPSDAPULLUPEN` writer - REG_KP_SDA_PULLUP_EN"]
pub type RegkpsdapullupenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGDDRKPSDAPULLUPEN` reader - REG_DDR_KP_SDA_PULLUP_EN"]
pub type RegddrkpsdapullupenR = crate::FieldReader;
#[doc = "Field `REGDDRKPSDAPULLUPEN` writer - REG_DDR_KP_SDA_PULLUP_EN"]
pub type RegddrkpsdapullupenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - REG_IDLE_SDA_PULLUP_EN"]
    #[inline(always)]
    pub fn regidlesdapullupen(&self) -> RegidlesdapullupenR {
        RegidlesdapullupenR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - REG_IDLE_SCL_PULLUP_EN"]
    #[inline(always)]
    pub fn regidlesclpullupen(&self) -> RegidlesclpullupenR {
        RegidlesclpullupenR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - REG_KP_SDA_PULLUP_EN"]
    #[inline(always)]
    pub fn regkpsdapullupen(&self) -> RegkpsdapullupenR {
        RegkpsdapullupenR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - REG_DDR_KP_SDA_PULLUP_EN"]
    #[inline(always)]
    pub fn regddrkpsdapullupen(&self) -> RegddrkpsdapullupenR {
        RegddrkpsdapullupenR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_IDLE_SDA_PULLUP_EN"]
    #[inline(always)]
    pub fn regidlesdapullupen(&mut self) -> RegidlesdapullupenW<I3cphyctrlreg0d0Spec> {
        RegidlesdapullupenW::new(self, 0)
    }
    #[doc = "Bits 4:6 - REG_IDLE_SCL_PULLUP_EN"]
    #[inline(always)]
    pub fn regidlesclpullupen(&mut self) -> RegidlesclpullupenW<I3cphyctrlreg0d0Spec> {
        RegidlesclpullupenW::new(self, 4)
    }
    #[doc = "Bits 8:10 - REG_KP_SDA_PULLUP_EN"]
    #[inline(always)]
    pub fn regkpsdapullupen(&mut self) -> RegkpsdapullupenW<I3cphyctrlreg0d0Spec> {
        RegkpsdapullupenW::new(self, 8)
    }
    #[doc = "Bits 12:14 - REG_DDR_KP_SDA_PULLUP_EN"]
    #[inline(always)]
    pub fn regddrkpsdapullupen(&mut self) -> RegddrkpsdapullupenW<I3cphyctrlreg0d0Spec> {
        RegddrkpsdapullupenW::new(self, 12)
    }
}
#[doc = "CR\\_SCL\\_SDA\\_PULLUP\\_EN\\_ADDITIONAL\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0d0Spec;
impl crate::RegisterSpec for I3cphyctrlreg0d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0d0::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0d0Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0d0::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0D0 to value 0"]
impl crate::Resettable for I3cphyctrlreg0d0Spec {}
