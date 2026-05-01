#[doc = "Register `SCUE4C` reader"]
pub type R = crate::R<Scue4cSpec>;
#[doc = "Register `SCUE4C` writer"]
pub type W = crate::W<Scue4cSpec>;
#[doc = "Field `SCUREGLOCK980` reader - SCU_REG_LOCK_980"]
pub type Scureglock980R = crate::BitReader;
#[doc = "Field `SCUREGLOCK980` writer - SCU_REG_LOCK_980"]
pub type Scureglock980W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK984` reader - SCU_REG_LOCK_984"]
pub type Scureglock984R = crate::BitReader;
#[doc = "Field `SCUREGLOCK984` writer - SCU_REG_LOCK_984"]
pub type Scureglock984W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK988` reader - SCU_REG_LOCK_988"]
pub type Scureglock988R = crate::BitReader;
#[doc = "Field `SCUREGLOCK988` writer - SCU_REG_LOCK_988"]
pub type Scureglock988W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK98C` reader - SCU_REG_LOCK_98C"]
pub type Scureglock98cR = crate::BitReader;
#[doc = "Field `SCUREGLOCK98C` writer - SCU_REG_LOCK_98C"]
pub type Scureglock98cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGLOCK9B0` reader - SCU_REG_LOCK_9B0"]
pub type Scureglock9b0R = crate::BitReader;
#[doc = "Field `SCUREGLOCK9B0` writer - SCU_REG_LOCK_9B0"]
pub type Scureglock9b0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK9B4` reader - SCU_REG_LOCK_9B4"]
pub type Scureglock9b4R = crate::BitReader;
#[doc = "Field `SCUREGLOCK9B4` writer - SCU_REG_LOCK_9B4"]
pub type Scureglock9b4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK9B8` reader - SCU_REG_LOCK_9B8"]
pub type Scureglock9b8R = crate::BitReader;
#[doc = "Field `SCUREGLOCK9B8` writer - SCU_REG_LOCK_9B8"]
pub type Scureglock9b8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK9BC` reader - SCU_REG_LOCK_9BC"]
pub type Scureglock9bcR = crate::BitReader;
#[doc = "Field `SCUREGLOCK9BC` writer - SCU_REG_LOCK_9BC"]
pub type Scureglock9bcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_LOCK_980"]
    #[inline(always)]
    pub fn scureglock980(&self) -> Scureglock980R {
        Scureglock980R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_984"]
    #[inline(always)]
    pub fn scureglock984(&self) -> Scureglock984R {
        Scureglock984R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_LOCK_988"]
    #[inline(always)]
    pub fn scureglock988(&self) -> Scureglock988R {
        Scureglock988R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_LOCK_98C"]
    #[inline(always)]
    pub fn scureglock98c(&self) -> Scureglock98cR {
        Scureglock98cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 12 - SCU_REG_LOCK_9B0"]
    #[inline(always)]
    pub fn scureglock9b0(&self) -> Scureglock9b0R {
        Scureglock9b0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_REG_LOCK_9B4"]
    #[inline(always)]
    pub fn scureglock9b4(&self) -> Scureglock9b4R {
        Scureglock9b4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCU_REG_LOCK_9B8"]
    #[inline(always)]
    pub fn scureglock9b8(&self) -> Scureglock9b8R {
        Scureglock9b8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SCU_REG_LOCK_9BC"]
    #[inline(always)]
    pub fn scureglock9bc(&self) -> Scureglock9bcR {
        Scureglock9bcR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_LOCK_980"]
    #[inline(always)]
    pub fn scureglock980(&mut self) -> Scureglock980W<Scue4cSpec> {
        Scureglock980W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_984"]
    #[inline(always)]
    pub fn scureglock984(&mut self) -> Scureglock984W<Scue4cSpec> {
        Scureglock984W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_REG_LOCK_988"]
    #[inline(always)]
    pub fn scureglock988(&mut self) -> Scureglock988W<Scue4cSpec> {
        Scureglock988W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_LOCK_98C"]
    #[inline(always)]
    pub fn scureglock98c(&mut self) -> Scureglock98cW<Scue4cSpec> {
        Scureglock98cW::new(self, 3)
    }
    #[doc = "Bit 12 - SCU_REG_LOCK_9B0"]
    #[inline(always)]
    pub fn scureglock9b0(&mut self) -> Scureglock9b0W<Scue4cSpec> {
        Scureglock9b0W::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_REG_LOCK_9B4"]
    #[inline(always)]
    pub fn scureglock9b4(&mut self) -> Scureglock9b4W<Scue4cSpec> {
        Scureglock9b4W::new(self, 13)
    }
    #[doc = "Bit 14 - SCU_REG_LOCK_9B8"]
    #[inline(always)]
    pub fn scureglock9b8(&mut self) -> Scureglock9b8W<Scue4cSpec> {
        Scureglock9b8W::new(self, 14)
    }
    #[doc = "Bit 15 - SCU_REG_LOCK_9BC"]
    #[inline(always)]
    pub fn scureglock9bc(&mut self) -> Scureglock9bcW<Scue4cSpec> {
        Scureglock9bcW::new(self, 15)
    }
}
#[doc = "Write Protection 20 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue4c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue4c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scue4cSpec;
impl crate::RegisterSpec for Scue4cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scue4c::R`](R) reader structure"]
impl crate::Readable for Scue4cSpec {}
#[doc = "`write(|w| ..)` method takes [`scue4c::W`](W) writer structure"]
impl crate::Writable for Scue4cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUE4C to value 0"]
impl crate::Resettable for Scue4cSpec {}
