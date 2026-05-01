#[doc = "Register `SCUC4C` reader"]
pub type R = crate::R<Scuc4cSpec>;
#[doc = "Register `SCUC4C` writer"]
pub type W = crate::W<Scuc4cSpec>;
#[doc = "Field `SCUREGSEC1980` reader - SCU_REG_SEC1_980"]
pub type Scuregsec1980R = crate::BitReader;
#[doc = "Field `SCUREGSEC1980` writer - SCU_REG_SEC1_980"]
pub type Scuregsec1980W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1984` reader - SCU_REG_SEC1_984"]
pub type Scuregsec1984R = crate::BitReader;
#[doc = "Field `SCUREGSEC1984` writer - SCU_REG_SEC1_984"]
pub type Scuregsec1984W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1988` reader - SCU_REG_SEC1_988"]
pub type Scuregsec1988R = crate::BitReader;
#[doc = "Field `SCUREGSEC1988` writer - SCU_REG_SEC1_988"]
pub type Scuregsec1988W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC198C` reader - SCU_REG_SEC1_98C"]
pub type Scuregsec198cR = crate::BitReader;
#[doc = "Field `SCUREGSEC198C` writer - SCU_REG_SEC1_98C"]
pub type Scuregsec198cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGSEC19B0` reader - SCU_REG_SEC1_9B0"]
pub type Scuregsec19b0R = crate::BitReader;
#[doc = "Field `SCUREGSEC19B0` writer - SCU_REG_SEC1_9B0"]
pub type Scuregsec19b0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC19B4` reader - SCU_REG_SEC1_9B4"]
pub type Scuregsec19b4R = crate::BitReader;
#[doc = "Field `SCUREGSEC19B4` writer - SCU_REG_SEC1_9B4"]
pub type Scuregsec19b4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC19B8` reader - SCU_REG_SEC1_9B8"]
pub type Scuregsec19b8R = crate::BitReader;
#[doc = "Field `SCUREGSEC19B8` writer - SCU_REG_SEC1_9B8"]
pub type Scuregsec19b8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC19BC` reader - SCU_REG_SEC1_9BC"]
pub type Scuregsec19bcR = crate::BitReader;
#[doc = "Field `SCUREGSEC19BC` writer - SCU_REG_SEC1_9BC"]
pub type Scuregsec19bcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_SEC1_980"]
    #[inline(always)]
    pub fn scuregsec1980(&self) -> Scuregsec1980R {
        Scuregsec1980R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC1_984"]
    #[inline(always)]
    pub fn scuregsec1984(&self) -> Scuregsec1984R {
        Scuregsec1984R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_SEC1_988"]
    #[inline(always)]
    pub fn scuregsec1988(&self) -> Scuregsec1988R {
        Scuregsec1988R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_SEC1_98C"]
    #[inline(always)]
    pub fn scuregsec198c(&self) -> Scuregsec198cR {
        Scuregsec198cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 12 - SCU_REG_SEC1_9B0"]
    #[inline(always)]
    pub fn scuregsec19b0(&self) -> Scuregsec19b0R {
        Scuregsec19b0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_REG_SEC1_9B4"]
    #[inline(always)]
    pub fn scuregsec19b4(&self) -> Scuregsec19b4R {
        Scuregsec19b4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCU_REG_SEC1_9B8"]
    #[inline(always)]
    pub fn scuregsec19b8(&self) -> Scuregsec19b8R {
        Scuregsec19b8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SCU_REG_SEC1_9BC"]
    #[inline(always)]
    pub fn scuregsec19bc(&self) -> Scuregsec19bcR {
        Scuregsec19bcR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_SEC1_980"]
    #[inline(always)]
    pub fn scuregsec1980(&mut self) -> Scuregsec1980W<Scuc4cSpec> {
        Scuregsec1980W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC1_984"]
    #[inline(always)]
    pub fn scuregsec1984(&mut self) -> Scuregsec1984W<Scuc4cSpec> {
        Scuregsec1984W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_REG_SEC1_988"]
    #[inline(always)]
    pub fn scuregsec1988(&mut self) -> Scuregsec1988W<Scuc4cSpec> {
        Scuregsec1988W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_SEC1_98C"]
    #[inline(always)]
    pub fn scuregsec198c(&mut self) -> Scuregsec198cW<Scuc4cSpec> {
        Scuregsec198cW::new(self, 3)
    }
    #[doc = "Bit 12 - SCU_REG_SEC1_9B0"]
    #[inline(always)]
    pub fn scuregsec19b0(&mut self) -> Scuregsec19b0W<Scuc4cSpec> {
        Scuregsec19b0W::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_REG_SEC1_9B4"]
    #[inline(always)]
    pub fn scuregsec19b4(&mut self) -> Scuregsec19b4W<Scuc4cSpec> {
        Scuregsec19b4W::new(self, 13)
    }
    #[doc = "Bit 14 - SCU_REG_SEC1_9B8"]
    #[inline(always)]
    pub fn scuregsec19b8(&mut self) -> Scuregsec19b8W<Scuc4cSpec> {
        Scuregsec19b8W::new(self, 14)
    }
    #[doc = "Bit 15 - SCU_REG_SEC1_9BC"]
    #[inline(always)]
    pub fn scuregsec19bc(&mut self) -> Scuregsec19bcW<Scuc4cSpec> {
        Scuregsec19bcW::new(self, 15)
    }
}
#[doc = "Secure1 Control 20 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc4c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc4c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuc4cSpec;
impl crate::RegisterSpec for Scuc4cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuc4c::R`](R) reader structure"]
impl crate::Readable for Scuc4cSpec {}
#[doc = "`write(|w| ..)` method takes [`scuc4c::W`](W) writer structure"]
impl crate::Writable for Scuc4cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUC4C to value 0"]
impl crate::Resettable for Scuc4cSpec {}
