#[doc = "Register `SCU2B4` reader"]
pub type R = crate::R<Scu2b4Spec>;
#[doc = "Register `SCU2B4` writer"]
pub type W = crate::W<Scu2b4Spec>;
#[doc = "Field `SCUCLKSEL2SEC10` reader - SCU_CLK_SEL2_SEC1_0"]
pub type Scuclksel2sec10R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC10` writer - SCU_CLK_SEL2_SEC1_0"]
pub type Scuclksel2sec10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2SEC13` reader - SCU_CLK_SEL2_SEC1_3"]
pub type Scuclksel2sec13R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC13` writer - SCU_CLK_SEL2_SEC1_3"]
pub type Scuclksel2sec13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC15` reader - SCU_CLK_SEL2_SEC1_5"]
pub type Scuclksel2sec15R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC15` writer - SCU_CLK_SEL2_SEC1_5"]
pub type Scuclksel2sec15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2SEC18` reader - SCU_CLK_SEL2_SEC1_8"]
pub type Scuclksel2sec18R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC18` writer - SCU_CLK_SEL2_SEC1_8"]
pub type Scuclksel2sec18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2SEC112` reader - SCU_CLK_SEL2_SEC1_12"]
pub type Scuclksel2sec112R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC112` writer - SCU_CLK_SEL2_SEC1_12"]
pub type Scuclksel2sec112W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2SEC115` reader - SCU_CLK_SEL2_SEC1_15"]
pub type Scuclksel2sec115R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC115` writer - SCU_CLK_SEL2_SEC1_15"]
pub type Scuclksel2sec115W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2SEC116` reader - SCU_CLK_SEL2_SEC1_16"]
pub type Scuclksel2sec116R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC116` writer - SCU_CLK_SEL2_SEC1_16"]
pub type Scuclksel2sec116W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2SEC117` reader - SCU_CLK_SEL2_SEC1_17"]
pub type Scuclksel2sec117R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC117` writer - SCU_CLK_SEL2_SEC1_17"]
pub type Scuclksel2sec117W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2SEC118` reader - SCU_CLK_SEL2_SEC1_18"]
pub type Scuclksel2sec118R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC118` writer - SCU_CLK_SEL2_SEC1_18"]
pub type Scuclksel2sec118W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2SEC119` reader - SCU_CLK_SEL2_SEC1_19"]
pub type Scuclksel2sec119R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC119` writer - SCU_CLK_SEL2_SEC1_19"]
pub type Scuclksel2sec119W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2SEC120` reader - SCU_CLK_SEL2_SEC1_20"]
pub type Scuclksel2sec120R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC120` writer - SCU_CLK_SEL2_SEC1_20"]
pub type Scuclksel2sec120W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2SEC123` reader - SCU_CLK_SEL2_SEC1_23"]
pub type Scuclksel2sec123R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC123` writer - SCU_CLK_SEL2_SEC1_23"]
pub type Scuclksel2sec123W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_CLK_SEL2_SEC1_0"]
    #[inline(always)]
    pub fn scuclksel2sec10(&self) -> Scuclksel2sec10R {
        Scuclksel2sec10R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - SCU_CLK_SEL2_SEC1_3"]
    #[inline(always)]
    pub fn scuclksel2sec13(&self) -> Scuclksel2sec13R {
        Scuclksel2sec13R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_CLK_SEL2_SEC1_5"]
    #[inline(always)]
    pub fn scuclksel2sec15(&self) -> Scuclksel2sec15R {
        Scuclksel2sec15R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SCU_CLK_SEL2_SEC1_8"]
    #[inline(always)]
    pub fn scuclksel2sec18(&self) -> Scuclksel2sec18R {
        Scuclksel2sec18R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - SCU_CLK_SEL2_SEC1_12"]
    #[inline(always)]
    pub fn scuclksel2sec112(&self) -> Scuclksel2sec112R {
        Scuclksel2sec112R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - SCU_CLK_SEL2_SEC1_15"]
    #[inline(always)]
    pub fn scuclksel2sec115(&self) -> Scuclksel2sec115R {
        Scuclksel2sec115R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCU_CLK_SEL2_SEC1_16"]
    #[inline(always)]
    pub fn scuclksel2sec116(&self) -> Scuclksel2sec116R {
        Scuclksel2sec116R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_CLK_SEL2_SEC1_17"]
    #[inline(always)]
    pub fn scuclksel2sec117(&self) -> Scuclksel2sec117R {
        Scuclksel2sec117R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_CLK_SEL2_SEC1_18"]
    #[inline(always)]
    pub fn scuclksel2sec118(&self) -> Scuclksel2sec118R {
        Scuclksel2sec118R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCU_CLK_SEL2_SEC1_19"]
    #[inline(always)]
    pub fn scuclksel2sec119(&self) -> Scuclksel2sec119R {
        Scuclksel2sec119R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SCU_CLK_SEL2_SEC1_20"]
    #[inline(always)]
    pub fn scuclksel2sec120(&self) -> Scuclksel2sec120R {
        Scuclksel2sec120R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - SCU_CLK_SEL2_SEC1_23"]
    #[inline(always)]
    pub fn scuclksel2sec123(&self) -> Scuclksel2sec123R {
        Scuclksel2sec123R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_CLK_SEL2_SEC1_0"]
    #[inline(always)]
    pub fn scuclksel2sec10(&mut self) -> Scuclksel2sec10W<Scu2b4Spec> {
        Scuclksel2sec10W::new(self, 0)
    }
    #[doc = "Bit 3 - SCU_CLK_SEL2_SEC1_3"]
    #[inline(always)]
    pub fn scuclksel2sec13(&mut self) -> Scuclksel2sec13W<Scu2b4Spec> {
        Scuclksel2sec13W::new(self, 3)
    }
    #[doc = "Bit 5 - SCU_CLK_SEL2_SEC1_5"]
    #[inline(always)]
    pub fn scuclksel2sec15(&mut self) -> Scuclksel2sec15W<Scu2b4Spec> {
        Scuclksel2sec15W::new(self, 5)
    }
    #[doc = "Bit 8 - SCU_CLK_SEL2_SEC1_8"]
    #[inline(always)]
    pub fn scuclksel2sec18(&mut self) -> Scuclksel2sec18W<Scu2b4Spec> {
        Scuclksel2sec18W::new(self, 8)
    }
    #[doc = "Bit 12 - SCU_CLK_SEL2_SEC1_12"]
    #[inline(always)]
    pub fn scuclksel2sec112(&mut self) -> Scuclksel2sec112W<Scu2b4Spec> {
        Scuclksel2sec112W::new(self, 12)
    }
    #[doc = "Bit 15 - SCU_CLK_SEL2_SEC1_15"]
    #[inline(always)]
    pub fn scuclksel2sec115(&mut self) -> Scuclksel2sec115W<Scu2b4Spec> {
        Scuclksel2sec115W::new(self, 15)
    }
    #[doc = "Bit 16 - SCU_CLK_SEL2_SEC1_16"]
    #[inline(always)]
    pub fn scuclksel2sec116(&mut self) -> Scuclksel2sec116W<Scu2b4Spec> {
        Scuclksel2sec116W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_CLK_SEL2_SEC1_17"]
    #[inline(always)]
    pub fn scuclksel2sec117(&mut self) -> Scuclksel2sec117W<Scu2b4Spec> {
        Scuclksel2sec117W::new(self, 17)
    }
    #[doc = "Bit 18 - SCU_CLK_SEL2_SEC1_18"]
    #[inline(always)]
    pub fn scuclksel2sec118(&mut self) -> Scuclksel2sec118W<Scu2b4Spec> {
        Scuclksel2sec118W::new(self, 18)
    }
    #[doc = "Bit 19 - SCU_CLK_SEL2_SEC1_19"]
    #[inline(always)]
    pub fn scuclksel2sec119(&mut self) -> Scuclksel2sec119W<Scu2b4Spec> {
        Scuclksel2sec119W::new(self, 19)
    }
    #[doc = "Bit 20 - SCU_CLK_SEL2_SEC1_20"]
    #[inline(always)]
    pub fn scuclksel2sec120(&mut self) -> Scuclksel2sec120W<Scu2b4Spec> {
        Scuclksel2sec120W::new(self, 20)
    }
    #[doc = "Bit 23 - SCU_CLK_SEL2_SEC1_23"]
    #[inline(always)]
    pub fn scuclksel2sec123(&mut self) -> Scuclksel2sec123W<Scu2b4Spec> {
        Scuclksel2sec123W::new(self, 23)
    }
}
#[doc = "Clock Selection Secure 2 Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu2b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu2b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu2b4Spec;
impl crate::RegisterSpec for Scu2b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu2b4::R`](R) reader structure"]
impl crate::Readable for Scu2b4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu2b4::W`](W) writer structure"]
impl crate::Writable for Scu2b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU2B4 to value 0"]
impl crate::Resettable for Scu2b4Spec {}
