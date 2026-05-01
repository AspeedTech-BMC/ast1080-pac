#[doc = "Register `SCU2B8` reader"]
pub type R = crate::R<Scu2b8Spec>;
#[doc = "Register `SCU2B8` writer"]
pub type W = crate::W<Scu2b8Spec>;
#[doc = "Field `SCUCLKSEL2SEC20` reader - SCU_CLK_SEL2_SEC2_0"]
pub type Scuclksel2sec20R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC20` writer - SCU_CLK_SEL2_SEC2_0"]
pub type Scuclksel2sec20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2SEC23` reader - SCU_CLK_SEL2_SEC2_3"]
pub type Scuclksel2sec23R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC23` writer - SCU_CLK_SEL2_SEC2_3"]
pub type Scuclksel2sec23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC25` reader - SCU_CLK_SEL2_SEC2_5"]
pub type Scuclksel2sec25R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC25` writer - SCU_CLK_SEL2_SEC2_5"]
pub type Scuclksel2sec25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2SEC28` reader - SCU_CLK_SEL2_SEC2_8"]
pub type Scuclksel2sec28R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC28` writer - SCU_CLK_SEL2_SEC2_8"]
pub type Scuclksel2sec28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2SEC212` reader - SCU_CLK_SEL2_SEC2_12"]
pub type Scuclksel2sec212R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC212` writer - SCU_CLK_SEL2_SEC2_12"]
pub type Scuclksel2sec212W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2SEC215` reader - SCU_CLK_SEL2_SEC2_15"]
pub type Scuclksel2sec215R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC215` writer - SCU_CLK_SEL2_SEC2_15"]
pub type Scuclksel2sec215W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2SEC216` reader - SCU_CLK_SEL2_SEC2_16"]
pub type Scuclksel2sec216R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC216` writer - SCU_CLK_SEL2_SEC2_16"]
pub type Scuclksel2sec216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2SEC217` reader - SCU_CLK_SEL2_SEC2_17"]
pub type Scuclksel2sec217R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC217` writer - SCU_CLK_SEL2_SEC2_17"]
pub type Scuclksel2sec217W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2SEC218` reader - SCU_CLK_SEL2_SEC2_18"]
pub type Scuclksel2sec218R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC218` writer - SCU_CLK_SEL2_SEC2_18"]
pub type Scuclksel2sec218W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2SEC219` reader - SCU_CLK_SEL2_SEC2_19"]
pub type Scuclksel2sec219R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC219` writer - SCU_CLK_SEL2_SEC2_19"]
pub type Scuclksel2sec219W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2SEC220` reader - SCU_CLK_SEL2_SEC2_20"]
pub type Scuclksel2sec220R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC220` writer - SCU_CLK_SEL2_SEC2_20"]
pub type Scuclksel2sec220W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2SEC223` reader - SCU_CLK_SEL2_SEC2_23"]
pub type Scuclksel2sec223R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC223` writer - SCU_CLK_SEL2_SEC2_23"]
pub type Scuclksel2sec223W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_CLK_SEL2_SEC2_0"]
    #[inline(always)]
    pub fn scuclksel2sec20(&self) -> Scuclksel2sec20R {
        Scuclksel2sec20R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - SCU_CLK_SEL2_SEC2_3"]
    #[inline(always)]
    pub fn scuclksel2sec23(&self) -> Scuclksel2sec23R {
        Scuclksel2sec23R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_CLK_SEL2_SEC2_5"]
    #[inline(always)]
    pub fn scuclksel2sec25(&self) -> Scuclksel2sec25R {
        Scuclksel2sec25R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SCU_CLK_SEL2_SEC2_8"]
    #[inline(always)]
    pub fn scuclksel2sec28(&self) -> Scuclksel2sec28R {
        Scuclksel2sec28R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - SCU_CLK_SEL2_SEC2_12"]
    #[inline(always)]
    pub fn scuclksel2sec212(&self) -> Scuclksel2sec212R {
        Scuclksel2sec212R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - SCU_CLK_SEL2_SEC2_15"]
    #[inline(always)]
    pub fn scuclksel2sec215(&self) -> Scuclksel2sec215R {
        Scuclksel2sec215R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCU_CLK_SEL2_SEC2_16"]
    #[inline(always)]
    pub fn scuclksel2sec216(&self) -> Scuclksel2sec216R {
        Scuclksel2sec216R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_CLK_SEL2_SEC2_17"]
    #[inline(always)]
    pub fn scuclksel2sec217(&self) -> Scuclksel2sec217R {
        Scuclksel2sec217R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_CLK_SEL2_SEC2_18"]
    #[inline(always)]
    pub fn scuclksel2sec218(&self) -> Scuclksel2sec218R {
        Scuclksel2sec218R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCU_CLK_SEL2_SEC2_19"]
    #[inline(always)]
    pub fn scuclksel2sec219(&self) -> Scuclksel2sec219R {
        Scuclksel2sec219R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SCU_CLK_SEL2_SEC2_20"]
    #[inline(always)]
    pub fn scuclksel2sec220(&self) -> Scuclksel2sec220R {
        Scuclksel2sec220R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - SCU_CLK_SEL2_SEC2_23"]
    #[inline(always)]
    pub fn scuclksel2sec223(&self) -> Scuclksel2sec223R {
        Scuclksel2sec223R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_CLK_SEL2_SEC2_0"]
    #[inline(always)]
    pub fn scuclksel2sec20(&mut self) -> Scuclksel2sec20W<Scu2b8Spec> {
        Scuclksel2sec20W::new(self, 0)
    }
    #[doc = "Bit 3 - SCU_CLK_SEL2_SEC2_3"]
    #[inline(always)]
    pub fn scuclksel2sec23(&mut self) -> Scuclksel2sec23W<Scu2b8Spec> {
        Scuclksel2sec23W::new(self, 3)
    }
    #[doc = "Bit 5 - SCU_CLK_SEL2_SEC2_5"]
    #[inline(always)]
    pub fn scuclksel2sec25(&mut self) -> Scuclksel2sec25W<Scu2b8Spec> {
        Scuclksel2sec25W::new(self, 5)
    }
    #[doc = "Bit 8 - SCU_CLK_SEL2_SEC2_8"]
    #[inline(always)]
    pub fn scuclksel2sec28(&mut self) -> Scuclksel2sec28W<Scu2b8Spec> {
        Scuclksel2sec28W::new(self, 8)
    }
    #[doc = "Bit 12 - SCU_CLK_SEL2_SEC2_12"]
    #[inline(always)]
    pub fn scuclksel2sec212(&mut self) -> Scuclksel2sec212W<Scu2b8Spec> {
        Scuclksel2sec212W::new(self, 12)
    }
    #[doc = "Bit 15 - SCU_CLK_SEL2_SEC2_15"]
    #[inline(always)]
    pub fn scuclksel2sec215(&mut self) -> Scuclksel2sec215W<Scu2b8Spec> {
        Scuclksel2sec215W::new(self, 15)
    }
    #[doc = "Bit 16 - SCU_CLK_SEL2_SEC2_16"]
    #[inline(always)]
    pub fn scuclksel2sec216(&mut self) -> Scuclksel2sec216W<Scu2b8Spec> {
        Scuclksel2sec216W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_CLK_SEL2_SEC2_17"]
    #[inline(always)]
    pub fn scuclksel2sec217(&mut self) -> Scuclksel2sec217W<Scu2b8Spec> {
        Scuclksel2sec217W::new(self, 17)
    }
    #[doc = "Bit 18 - SCU_CLK_SEL2_SEC2_18"]
    #[inline(always)]
    pub fn scuclksel2sec218(&mut self) -> Scuclksel2sec218W<Scu2b8Spec> {
        Scuclksel2sec218W::new(self, 18)
    }
    #[doc = "Bit 19 - SCU_CLK_SEL2_SEC2_19"]
    #[inline(always)]
    pub fn scuclksel2sec219(&mut self) -> Scuclksel2sec219W<Scu2b8Spec> {
        Scuclksel2sec219W::new(self, 19)
    }
    #[doc = "Bit 20 - SCU_CLK_SEL2_SEC2_20"]
    #[inline(always)]
    pub fn scuclksel2sec220(&mut self) -> Scuclksel2sec220W<Scu2b8Spec> {
        Scuclksel2sec220W::new(self, 20)
    }
    #[doc = "Bit 23 - SCU_CLK_SEL2_SEC2_23"]
    #[inline(always)]
    pub fn scuclksel2sec223(&mut self) -> Scuclksel2sec223W<Scu2b8Spec> {
        Scuclksel2sec223W::new(self, 23)
    }
}
#[doc = "Clock Selection Secure 2 Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu2b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu2b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu2b8Spec;
impl crate::RegisterSpec for Scu2b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu2b8::R`](R) reader structure"]
impl crate::Readable for Scu2b8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu2b8::W`](W) writer structure"]
impl crate::Writable for Scu2b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU2B8 to value 0"]
impl crate::Resettable for Scu2b8Spec {}
