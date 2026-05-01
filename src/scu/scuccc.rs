#[doc = "Register `SCUCCC` reader"]
pub type R = crate::R<ScucccSpec>;
#[doc = "Register `SCUCCC` writer"]
pub type W = crate::W<ScucccSpec>;
#[doc = "Field `SCUREGSEC2980` reader - SCU_REG_SEC2_980"]
pub type Scuregsec2980R = crate::BitReader;
#[doc = "Field `SCUREGSEC2980` writer - SCU_REG_SEC2_980"]
pub type Scuregsec2980W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2984` reader - SCU_REG_SEC2_984"]
pub type Scuregsec2984R = crate::BitReader;
#[doc = "Field `SCUREGSEC2984` writer - SCU_REG_SEC2_984"]
pub type Scuregsec2984W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2988` reader - SCU_REG_SEC2_988"]
pub type Scuregsec2988R = crate::BitReader;
#[doc = "Field `SCUREGSEC2988` writer - SCU_REG_SEC2_988"]
pub type Scuregsec2988W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC298C` reader - SCU_REG_SEC2_98C"]
pub type Scuregsec298cR = crate::BitReader;
#[doc = "Field `SCUREGSEC298C` writer - SCU_REG_SEC2_98C"]
pub type Scuregsec298cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGSEC29B0` reader - SCU_REG_SEC2_9B0"]
pub type Scuregsec29b0R = crate::BitReader;
#[doc = "Field `SCUREGSEC29B0` writer - SCU_REG_SEC2_9B0"]
pub type Scuregsec29b0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC29B4` reader - SCU_REG_SEC2_9B4"]
pub type Scuregsec29b4R = crate::BitReader;
#[doc = "Field `SCUREGSEC29B4` writer - SCU_REG_SEC2_9B4"]
pub type Scuregsec29b4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC29B8` reader - SCU_REG_SEC2_9B8"]
pub type Scuregsec29b8R = crate::BitReader;
#[doc = "Field `SCUREGSEC29B8` writer - SCU_REG_SEC2_9B8"]
pub type Scuregsec29b8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC29BC` reader - SCU_REG_SEC2_9BC"]
pub type Scuregsec29bcR = crate::BitReader;
#[doc = "Field `SCUREGSEC29BC` writer - SCU_REG_SEC2_9BC"]
pub type Scuregsec29bcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_SEC2_980"]
    #[inline(always)]
    pub fn scuregsec2980(&self) -> Scuregsec2980R {
        Scuregsec2980R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC2_984"]
    #[inline(always)]
    pub fn scuregsec2984(&self) -> Scuregsec2984R {
        Scuregsec2984R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_SEC2_988"]
    #[inline(always)]
    pub fn scuregsec2988(&self) -> Scuregsec2988R {
        Scuregsec2988R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_SEC2_98C"]
    #[inline(always)]
    pub fn scuregsec298c(&self) -> Scuregsec298cR {
        Scuregsec298cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 12 - SCU_REG_SEC2_9B0"]
    #[inline(always)]
    pub fn scuregsec29b0(&self) -> Scuregsec29b0R {
        Scuregsec29b0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_REG_SEC2_9B4"]
    #[inline(always)]
    pub fn scuregsec29b4(&self) -> Scuregsec29b4R {
        Scuregsec29b4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCU_REG_SEC2_9B8"]
    #[inline(always)]
    pub fn scuregsec29b8(&self) -> Scuregsec29b8R {
        Scuregsec29b8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SCU_REG_SEC2_9BC"]
    #[inline(always)]
    pub fn scuregsec29bc(&self) -> Scuregsec29bcR {
        Scuregsec29bcR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_SEC2_980"]
    #[inline(always)]
    pub fn scuregsec2980(&mut self) -> Scuregsec2980W<ScucccSpec> {
        Scuregsec2980W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC2_984"]
    #[inline(always)]
    pub fn scuregsec2984(&mut self) -> Scuregsec2984W<ScucccSpec> {
        Scuregsec2984W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_REG_SEC2_988"]
    #[inline(always)]
    pub fn scuregsec2988(&mut self) -> Scuregsec2988W<ScucccSpec> {
        Scuregsec2988W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_SEC2_98C"]
    #[inline(always)]
    pub fn scuregsec298c(&mut self) -> Scuregsec298cW<ScucccSpec> {
        Scuregsec298cW::new(self, 3)
    }
    #[doc = "Bit 12 - SCU_REG_SEC2_9B0"]
    #[inline(always)]
    pub fn scuregsec29b0(&mut self) -> Scuregsec29b0W<ScucccSpec> {
        Scuregsec29b0W::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_REG_SEC2_9B4"]
    #[inline(always)]
    pub fn scuregsec29b4(&mut self) -> Scuregsec29b4W<ScucccSpec> {
        Scuregsec29b4W::new(self, 13)
    }
    #[doc = "Bit 14 - SCU_REG_SEC2_9B8"]
    #[inline(always)]
    pub fn scuregsec29b8(&mut self) -> Scuregsec29b8W<ScucccSpec> {
        Scuregsec29b8W::new(self, 14)
    }
    #[doc = "Bit 15 - SCU_REG_SEC2_9BC"]
    #[inline(always)]
    pub fn scuregsec29bc(&mut self) -> Scuregsec29bcW<ScucccSpec> {
        Scuregsec29bcW::new(self, 15)
    }
}
#[doc = "Secure2 Control 20 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuccc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuccc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScucccSpec;
impl crate::RegisterSpec for ScucccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuccc::R`](R) reader structure"]
impl crate::Readable for ScucccSpec {}
#[doc = "`write(|w| ..)` method takes [`scuccc::W`](W) writer structure"]
impl crate::Writable for ScucccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUCCC to value 0"]
impl crate::Resettable for ScucccSpec {}
