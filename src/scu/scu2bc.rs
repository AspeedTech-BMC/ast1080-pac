#[doc = "Register `SCU2BC` reader"]
pub type R = crate::R<Scu2bcSpec>;
#[doc = "Register `SCU2BC` writer"]
pub type W = crate::W<Scu2bcSpec>;
#[doc = "Field `SCUCLKSEL2SEC30` reader - SCU_CLK_SEL2_SEC3_0"]
pub type Scuclksel2sec30R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC30` writer - SCU_CLK_SEL2_SEC3_0"]
pub type Scuclksel2sec30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2SEC33` reader - SCU_CLK_SEL2_SEC3_3"]
pub type Scuclksel2sec33R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC33` writer - SCU_CLK_SEL2_SEC3_3"]
pub type Scuclksel2sec33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC35` reader - SCU_CLK_SEL2_SEC3_5"]
pub type Scuclksel2sec35R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC35` writer - SCU_CLK_SEL2_SEC3_5"]
pub type Scuclksel2sec35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2SEC38` reader - SCU_CLK_SEL2_SEC3_8"]
pub type Scuclksel2sec38R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC38` writer - SCU_CLK_SEL2_SEC3_8"]
pub type Scuclksel2sec38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2SEC312` reader - SCU_CLK_SEL2_SEC3_12"]
pub type Scuclksel2sec312R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC312` writer - SCU_CLK_SEL2_SEC3_12"]
pub type Scuclksel2sec312W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2SEC315` reader - SCU_CLK_SEL2_SEC3_15"]
pub type Scuclksel2sec315R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC315` writer - SCU_CLK_SEL2_SEC3_15"]
pub type Scuclksel2sec315W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2SEC316` reader - SCU_CLK_SEL2_SEC3_16"]
pub type Scuclksel2sec316R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC316` writer - SCU_CLK_SEL2_SEC3_16"]
pub type Scuclksel2sec316W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2SEC317` reader - SCU_CLK_SEL2_SEC3_17"]
pub type Scuclksel2sec317R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC317` writer - SCU_CLK_SEL2_SEC3_17"]
pub type Scuclksel2sec317W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2SEC318` reader - SCU_CLK_SEL2_SEC3_18"]
pub type Scuclksel2sec318R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC318` writer - SCU_CLK_SEL2_SEC3_18"]
pub type Scuclksel2sec318W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2SEC319` reader - SCU_CLK_SEL2_SEC3_19"]
pub type Scuclksel2sec319R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC319` writer - SCU_CLK_SEL2_SEC3_19"]
pub type Scuclksel2sec319W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSEL2SEC320` reader - SCU_CLK_SEL2_SEC3_20"]
pub type Scuclksel2sec320R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC320` writer - SCU_CLK_SEL2_SEC3_20"]
pub type Scuclksel2sec320W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUCLKSEL2SEC323` reader - SCU_CLK_SEL2_SEC3_23"]
pub type Scuclksel2sec323R = crate::BitReader;
#[doc = "Field `SCUCLKSEL2SEC323` writer - SCU_CLK_SEL2_SEC3_23"]
pub type Scuclksel2sec323W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_CLK_SEL2_SEC3_0"]
    #[inline(always)]
    pub fn scuclksel2sec30(&self) -> Scuclksel2sec30R {
        Scuclksel2sec30R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - SCU_CLK_SEL2_SEC3_3"]
    #[inline(always)]
    pub fn scuclksel2sec33(&self) -> Scuclksel2sec33R {
        Scuclksel2sec33R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_CLK_SEL2_SEC3_5"]
    #[inline(always)]
    pub fn scuclksel2sec35(&self) -> Scuclksel2sec35R {
        Scuclksel2sec35R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SCU_CLK_SEL2_SEC3_8"]
    #[inline(always)]
    pub fn scuclksel2sec38(&self) -> Scuclksel2sec38R {
        Scuclksel2sec38R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - SCU_CLK_SEL2_SEC3_12"]
    #[inline(always)]
    pub fn scuclksel2sec312(&self) -> Scuclksel2sec312R {
        Scuclksel2sec312R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - SCU_CLK_SEL2_SEC3_15"]
    #[inline(always)]
    pub fn scuclksel2sec315(&self) -> Scuclksel2sec315R {
        Scuclksel2sec315R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCU_CLK_SEL2_SEC3_16"]
    #[inline(always)]
    pub fn scuclksel2sec316(&self) -> Scuclksel2sec316R {
        Scuclksel2sec316R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_CLK_SEL2_SEC3_17"]
    #[inline(always)]
    pub fn scuclksel2sec317(&self) -> Scuclksel2sec317R {
        Scuclksel2sec317R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_CLK_SEL2_SEC3_18"]
    #[inline(always)]
    pub fn scuclksel2sec318(&self) -> Scuclksel2sec318R {
        Scuclksel2sec318R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCU_CLK_SEL2_SEC3_19"]
    #[inline(always)]
    pub fn scuclksel2sec319(&self) -> Scuclksel2sec319R {
        Scuclksel2sec319R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SCU_CLK_SEL2_SEC3_20"]
    #[inline(always)]
    pub fn scuclksel2sec320(&self) -> Scuclksel2sec320R {
        Scuclksel2sec320R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - SCU_CLK_SEL2_SEC3_23"]
    #[inline(always)]
    pub fn scuclksel2sec323(&self) -> Scuclksel2sec323R {
        Scuclksel2sec323R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_CLK_SEL2_SEC3_0"]
    #[inline(always)]
    pub fn scuclksel2sec30(&mut self) -> Scuclksel2sec30W<Scu2bcSpec> {
        Scuclksel2sec30W::new(self, 0)
    }
    #[doc = "Bit 3 - SCU_CLK_SEL2_SEC3_3"]
    #[inline(always)]
    pub fn scuclksel2sec33(&mut self) -> Scuclksel2sec33W<Scu2bcSpec> {
        Scuclksel2sec33W::new(self, 3)
    }
    #[doc = "Bit 5 - SCU_CLK_SEL2_SEC3_5"]
    #[inline(always)]
    pub fn scuclksel2sec35(&mut self) -> Scuclksel2sec35W<Scu2bcSpec> {
        Scuclksel2sec35W::new(self, 5)
    }
    #[doc = "Bit 8 - SCU_CLK_SEL2_SEC3_8"]
    #[inline(always)]
    pub fn scuclksel2sec38(&mut self) -> Scuclksel2sec38W<Scu2bcSpec> {
        Scuclksel2sec38W::new(self, 8)
    }
    #[doc = "Bit 12 - SCU_CLK_SEL2_SEC3_12"]
    #[inline(always)]
    pub fn scuclksel2sec312(&mut self) -> Scuclksel2sec312W<Scu2bcSpec> {
        Scuclksel2sec312W::new(self, 12)
    }
    #[doc = "Bit 15 - SCU_CLK_SEL2_SEC3_15"]
    #[inline(always)]
    pub fn scuclksel2sec315(&mut self) -> Scuclksel2sec315W<Scu2bcSpec> {
        Scuclksel2sec315W::new(self, 15)
    }
    #[doc = "Bit 16 - SCU_CLK_SEL2_SEC3_16"]
    #[inline(always)]
    pub fn scuclksel2sec316(&mut self) -> Scuclksel2sec316W<Scu2bcSpec> {
        Scuclksel2sec316W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_CLK_SEL2_SEC3_17"]
    #[inline(always)]
    pub fn scuclksel2sec317(&mut self) -> Scuclksel2sec317W<Scu2bcSpec> {
        Scuclksel2sec317W::new(self, 17)
    }
    #[doc = "Bit 18 - SCU_CLK_SEL2_SEC3_18"]
    #[inline(always)]
    pub fn scuclksel2sec318(&mut self) -> Scuclksel2sec318W<Scu2bcSpec> {
        Scuclksel2sec318W::new(self, 18)
    }
    #[doc = "Bit 19 - SCU_CLK_SEL2_SEC3_19"]
    #[inline(always)]
    pub fn scuclksel2sec319(&mut self) -> Scuclksel2sec319W<Scu2bcSpec> {
        Scuclksel2sec319W::new(self, 19)
    }
    #[doc = "Bit 20 - SCU_CLK_SEL2_SEC3_20"]
    #[inline(always)]
    pub fn scuclksel2sec320(&mut self) -> Scuclksel2sec320W<Scu2bcSpec> {
        Scuclksel2sec320W::new(self, 20)
    }
    #[doc = "Bit 23 - SCU_CLK_SEL2_SEC3_23"]
    #[inline(always)]
    pub fn scuclksel2sec323(&mut self) -> Scuclksel2sec323W<Scu2bcSpec> {
        Scuclksel2sec323W::new(self, 23)
    }
}
#[doc = "Clock Selection Secure 2 Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu2bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu2bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu2bcSpec;
impl crate::RegisterSpec for Scu2bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu2bc::R`](R) reader structure"]
impl crate::Readable for Scu2bcSpec {}
#[doc = "`write(|w| ..)` method takes [`scu2bc::W`](W) writer structure"]
impl crate::Writable for Scu2bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU2BC to value 0"]
impl crate::Resettable for Scu2bcSpec {}
