#[doc = "Register `SCUD4C` reader"]
pub type R = crate::R<Scud4cSpec>;
#[doc = "Register `SCUD4C` writer"]
pub type W = crate::W<Scud4cSpec>;
#[doc = "Field `SCUREGSEC3980` reader - SCU_REG_SEC3_980"]
pub type Scuregsec3980R = crate::BitReader;
#[doc = "Field `SCUREGSEC3980` writer - SCU_REG_SEC3_980"]
pub type Scuregsec3980W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3984` reader - SCU_REG_SEC3_984"]
pub type Scuregsec3984R = crate::BitReader;
#[doc = "Field `SCUREGSEC3984` writer - SCU_REG_SEC3_984"]
pub type Scuregsec3984W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3988` reader - SCU_REG_SEC3_988"]
pub type Scuregsec3988R = crate::BitReader;
#[doc = "Field `SCUREGSEC3988` writer - SCU_REG_SEC3_988"]
pub type Scuregsec3988W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC398C` reader - SCU_REG_SEC3_98C"]
pub type Scuregsec398cR = crate::BitReader;
#[doc = "Field `SCUREGSEC398C` writer - SCU_REG_SEC3_98C"]
pub type Scuregsec398cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGSEC39B0` reader - SCU_REG_SEC3_9B0"]
pub type Scuregsec39b0R = crate::BitReader;
#[doc = "Field `SCUREGSEC39B0` writer - SCU_REG_SEC3_9B0"]
pub type Scuregsec39b0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC39B4` reader - SCU_REG_SEC3_9B4"]
pub type Scuregsec39b4R = crate::BitReader;
#[doc = "Field `SCUREGSEC39B4` writer - SCU_REG_SEC3_9B4"]
pub type Scuregsec39b4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC39B8` reader - SCU_REG_SEC3_9B8"]
pub type Scuregsec39b8R = crate::BitReader;
#[doc = "Field `SCUREGSEC39B8` writer - SCU_REG_SEC3_9B8"]
pub type Scuregsec39b8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC39BC` reader - SCU_REG_SEC3_9BC"]
pub type Scuregsec39bcR = crate::BitReader;
#[doc = "Field `SCUREGSEC39BC` writer - SCU_REG_SEC3_9BC"]
pub type Scuregsec39bcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_SEC3_980"]
    #[inline(always)]
    pub fn scuregsec3980(&self) -> Scuregsec3980R {
        Scuregsec3980R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC3_984"]
    #[inline(always)]
    pub fn scuregsec3984(&self) -> Scuregsec3984R {
        Scuregsec3984R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_SEC3_988"]
    #[inline(always)]
    pub fn scuregsec3988(&self) -> Scuregsec3988R {
        Scuregsec3988R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_SEC3_98C"]
    #[inline(always)]
    pub fn scuregsec398c(&self) -> Scuregsec398cR {
        Scuregsec398cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 12 - SCU_REG_SEC3_9B0"]
    #[inline(always)]
    pub fn scuregsec39b0(&self) -> Scuregsec39b0R {
        Scuregsec39b0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_REG_SEC3_9B4"]
    #[inline(always)]
    pub fn scuregsec39b4(&self) -> Scuregsec39b4R {
        Scuregsec39b4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCU_REG_SEC3_9B8"]
    #[inline(always)]
    pub fn scuregsec39b8(&self) -> Scuregsec39b8R {
        Scuregsec39b8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SCU_REG_SEC3_9BC"]
    #[inline(always)]
    pub fn scuregsec39bc(&self) -> Scuregsec39bcR {
        Scuregsec39bcR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_SEC3_980"]
    #[inline(always)]
    pub fn scuregsec3980(&mut self) -> Scuregsec3980W<Scud4cSpec> {
        Scuregsec3980W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC3_984"]
    #[inline(always)]
    pub fn scuregsec3984(&mut self) -> Scuregsec3984W<Scud4cSpec> {
        Scuregsec3984W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_REG_SEC3_988"]
    #[inline(always)]
    pub fn scuregsec3988(&mut self) -> Scuregsec3988W<Scud4cSpec> {
        Scuregsec3988W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_SEC3_98C"]
    #[inline(always)]
    pub fn scuregsec398c(&mut self) -> Scuregsec398cW<Scud4cSpec> {
        Scuregsec398cW::new(self, 3)
    }
    #[doc = "Bit 12 - SCU_REG_SEC3_9B0"]
    #[inline(always)]
    pub fn scuregsec39b0(&mut self) -> Scuregsec39b0W<Scud4cSpec> {
        Scuregsec39b0W::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_REG_SEC3_9B4"]
    #[inline(always)]
    pub fn scuregsec39b4(&mut self) -> Scuregsec39b4W<Scud4cSpec> {
        Scuregsec39b4W::new(self, 13)
    }
    #[doc = "Bit 14 - SCU_REG_SEC3_9B8"]
    #[inline(always)]
    pub fn scuregsec39b8(&mut self) -> Scuregsec39b8W<Scud4cSpec> {
        Scuregsec39b8W::new(self, 14)
    }
    #[doc = "Bit 15 - SCU_REG_SEC3_9BC"]
    #[inline(always)]
    pub fn scuregsec39bc(&mut self) -> Scuregsec39bcW<Scud4cSpec> {
        Scuregsec39bcW::new(self, 15)
    }
}
#[doc = "Secure3 Control 20 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud4c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud4c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scud4cSpec;
impl crate::RegisterSpec for Scud4cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scud4c::R`](R) reader structure"]
impl crate::Readable for Scud4cSpec {}
#[doc = "`write(|w| ..)` method takes [`scud4c::W`](W) writer structure"]
impl crate::Writable for Scud4cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUD4C to value 0"]
impl crate::Resettable for Scud4cSpec {}
