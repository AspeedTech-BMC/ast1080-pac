#[doc = "Register `SCUF4C` reader"]
pub type R = crate::R<Scuf4cSpec>;
#[doc = "Register `SCUF4C` writer"]
pub type W = crate::W<Scuf4cSpec>;
#[doc = "Field `SCUREGRST980` reader - SCU_REG_RST_980"]
pub type Scuregrst980R = crate::BitReader;
#[doc = "Field `SCUREGRST980` writer - SCU_REG_RST_980"]
pub type Scuregrst980W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST984` reader - SCU_REG_RST_984"]
pub type Scuregrst984R = crate::BitReader;
#[doc = "Field `SCUREGRST984` writer - SCU_REG_RST_984"]
pub type Scuregrst984W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST988` reader - SCU_REG_RST_988"]
pub type Scuregrst988R = crate::BitReader;
#[doc = "Field `SCUREGRST988` writer - SCU_REG_RST_988"]
pub type Scuregrst988W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST98C` reader - SCU_REG_RST_98C"]
pub type Scuregrst98cR = crate::BitReader;
#[doc = "Field `SCUREGRST98C` writer - SCU_REG_RST_98C"]
pub type Scuregrst98cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGRST9B0` reader - SCU_REG_RST_9B0"]
pub type Scuregrst9b0R = crate::BitReader;
#[doc = "Field `SCUREGRST9B0` writer - SCU_REG_RST_9B0"]
pub type Scuregrst9b0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST9B4` reader - SCU_REG_RST_9B4"]
pub type Scuregrst9b4R = crate::BitReader;
#[doc = "Field `SCUREGRST9B4` writer - SCU_REG_RST_9B4"]
pub type Scuregrst9b4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST9B8` reader - SCU_REG_RST_9B8"]
pub type Scuregrst9b8R = crate::BitReader;
#[doc = "Field `SCUREGRST9B8` writer - SCU_REG_RST_9B8"]
pub type Scuregrst9b8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST9BC` reader - SCU_REG_RST_9BC"]
pub type Scuregrst9bcR = crate::BitReader;
#[doc = "Field `SCUREGRST9BC` writer - SCU_REG_RST_9BC"]
pub type Scuregrst9bcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST9FC` reader - SCU_REG_RST_9FC"]
pub type Scuregrst9fcR = crate::BitReader;
#[doc = "Field `SCUREGRST9FC` writer - SCU_REG_RST_9FC"]
pub type Scuregrst9fcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_RST_980"]
    #[inline(always)]
    pub fn scuregrst980(&self) -> Scuregrst980R {
        Scuregrst980R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_RST_984"]
    #[inline(always)]
    pub fn scuregrst984(&self) -> Scuregrst984R {
        Scuregrst984R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_RST_988"]
    #[inline(always)]
    pub fn scuregrst988(&self) -> Scuregrst988R {
        Scuregrst988R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_RST_98C"]
    #[inline(always)]
    pub fn scuregrst98c(&self) -> Scuregrst98cR {
        Scuregrst98cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 12 - SCU_REG_RST_9B0"]
    #[inline(always)]
    pub fn scuregrst9b0(&self) -> Scuregrst9b0R {
        Scuregrst9b0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_REG_RST_9B4"]
    #[inline(always)]
    pub fn scuregrst9b4(&self) -> Scuregrst9b4R {
        Scuregrst9b4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCU_REG_RST_9B8"]
    #[inline(always)]
    pub fn scuregrst9b8(&self) -> Scuregrst9b8R {
        Scuregrst9b8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SCU_REG_RST_9BC"]
    #[inline(always)]
    pub fn scuregrst9bc(&self) -> Scuregrst9bcR {
        Scuregrst9bcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - SCU_REG_RST_9FC"]
    #[inline(always)]
    pub fn scuregrst9fc(&self) -> Scuregrst9fcR {
        Scuregrst9fcR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_RST_980"]
    #[inline(always)]
    pub fn scuregrst980(&mut self) -> Scuregrst980W<Scuf4cSpec> {
        Scuregrst980W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_RST_984"]
    #[inline(always)]
    pub fn scuregrst984(&mut self) -> Scuregrst984W<Scuf4cSpec> {
        Scuregrst984W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_REG_RST_988"]
    #[inline(always)]
    pub fn scuregrst988(&mut self) -> Scuregrst988W<Scuf4cSpec> {
        Scuregrst988W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_RST_98C"]
    #[inline(always)]
    pub fn scuregrst98c(&mut self) -> Scuregrst98cW<Scuf4cSpec> {
        Scuregrst98cW::new(self, 3)
    }
    #[doc = "Bit 12 - SCU_REG_RST_9B0"]
    #[inline(always)]
    pub fn scuregrst9b0(&mut self) -> Scuregrst9b0W<Scuf4cSpec> {
        Scuregrst9b0W::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_REG_RST_9B4"]
    #[inline(always)]
    pub fn scuregrst9b4(&mut self) -> Scuregrst9b4W<Scuf4cSpec> {
        Scuregrst9b4W::new(self, 13)
    }
    #[doc = "Bit 14 - SCU_REG_RST_9B8"]
    #[inline(always)]
    pub fn scuregrst9b8(&mut self) -> Scuregrst9b8W<Scuf4cSpec> {
        Scuregrst9b8W::new(self, 14)
    }
    #[doc = "Bit 15 - SCU_REG_RST_9BC"]
    #[inline(always)]
    pub fn scuregrst9bc(&mut self) -> Scuregrst9bcW<Scuf4cSpec> {
        Scuregrst9bcW::new(self, 15)
    }
    #[doc = "Bit 31 - SCU_REG_RST_9FC"]
    #[inline(always)]
    pub fn scuregrst9fc(&mut self) -> Scuregrst9fcW<Scuf4cSpec> {
        Scuregrst9fcW::new(self, 31)
    }
}
#[doc = "Reset Control 20 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf4c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf4c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf4cSpec;
impl crate::RegisterSpec for Scuf4cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf4c::R`](R) reader structure"]
impl crate::Readable for Scuf4cSpec {}
#[doc = "`write(|w| ..)` method takes [`scuf4c::W`](W) writer structure"]
impl crate::Writable for Scuf4cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF4C to value 0"]
impl crate::Resettable for Scuf4cSpec {}
