#[doc = "Register `SCU44C` reader"]
pub type R = crate::R<Scu44cSpec>;
#[doc = "Register `SCU44C` writer"]
pub type W = crate::W<Scu44cSpec>;
#[doc = "Field `SCUMUXIO152` reader - SCU_MUX_IO152"]
pub type Scumuxio152R = crate::FieldReader;
#[doc = "Field `SCUMUXIO152` writer - SCU_MUX_IO152"]
pub type Scumuxio152W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO153` reader - SCU_MUX_IO153"]
pub type Scumuxio153R = crate::FieldReader;
#[doc = "Field `SCUMUXIO153` writer - SCU_MUX_IO153"]
pub type Scumuxio153W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO154` reader - SCU_MUX_IO154"]
pub type Scumuxio154R = crate::FieldReader;
#[doc = "Field `SCUMUXIO154` writer - SCU_MUX_IO154"]
pub type Scumuxio154W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO155` reader - SCU_MUX_IO155"]
pub type Scumuxio155R = crate::FieldReader;
#[doc = "Field `SCUMUXIO155` writer - SCU_MUX_IO155"]
pub type Scumuxio155W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO156` reader - SCU_MUX_IO156"]
pub type Scumuxio156R = crate::FieldReader;
#[doc = "Field `SCUMUXIO156` writer - SCU_MUX_IO156"]
pub type Scumuxio156W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO157` reader - SCU_MUX_IO157"]
pub type Scumuxio157R = crate::FieldReader;
#[doc = "Field `SCUMUXIO157` writer - SCU_MUX_IO157"]
pub type Scumuxio157W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO158` reader - SCU_MUX_IO158"]
pub type Scumuxio158R = crate::FieldReader;
#[doc = "Field `SCUMUXIO158` writer - SCU_MUX_IO158"]
pub type Scumuxio158W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO159` reader - SCU_MUX_IO159"]
pub type Scumuxio159R = crate::FieldReader;
#[doc = "Field `SCUMUXIO159` writer - SCU_MUX_IO159"]
pub type Scumuxio159W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO152"]
    #[inline(always)]
    pub fn scumuxio152(&self) -> Scumuxio152R {
        Scumuxio152R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO153"]
    #[inline(always)]
    pub fn scumuxio153(&self) -> Scumuxio153R {
        Scumuxio153R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO154"]
    #[inline(always)]
    pub fn scumuxio154(&self) -> Scumuxio154R {
        Scumuxio154R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO155"]
    #[inline(always)]
    pub fn scumuxio155(&self) -> Scumuxio155R {
        Scumuxio155R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO156"]
    #[inline(always)]
    pub fn scumuxio156(&self) -> Scumuxio156R {
        Scumuxio156R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO157"]
    #[inline(always)]
    pub fn scumuxio157(&self) -> Scumuxio157R {
        Scumuxio157R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO158"]
    #[inline(always)]
    pub fn scumuxio158(&self) -> Scumuxio158R {
        Scumuxio158R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO159"]
    #[inline(always)]
    pub fn scumuxio159(&self) -> Scumuxio159R {
        Scumuxio159R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO152"]
    #[inline(always)]
    pub fn scumuxio152(&mut self) -> Scumuxio152W<Scu44cSpec> {
        Scumuxio152W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO153"]
    #[inline(always)]
    pub fn scumuxio153(&mut self) -> Scumuxio153W<Scu44cSpec> {
        Scumuxio153W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO154"]
    #[inline(always)]
    pub fn scumuxio154(&mut self) -> Scumuxio154W<Scu44cSpec> {
        Scumuxio154W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO155"]
    #[inline(always)]
    pub fn scumuxio155(&mut self) -> Scumuxio155W<Scu44cSpec> {
        Scumuxio155W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO156"]
    #[inline(always)]
    pub fn scumuxio156(&mut self) -> Scumuxio156W<Scu44cSpec> {
        Scumuxio156W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO157"]
    #[inline(always)]
    pub fn scumuxio157(&mut self) -> Scumuxio157W<Scu44cSpec> {
        Scumuxio157W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO158"]
    #[inline(always)]
    pub fn scumuxio158(&mut self) -> Scumuxio158W<Scu44cSpec> {
        Scumuxio158W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO159"]
    #[inline(always)]
    pub fn scumuxio159(&mut self) -> Scumuxio159W<Scu44cSpec> {
        Scumuxio159W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#20\n\nYou can [`read`](crate::Reg::read) this register and get [`scu44c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu44c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu44cSpec;
impl crate::RegisterSpec for Scu44cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu44c::R`](R) reader structure"]
impl crate::Readable for Scu44cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu44c::W`](W) writer structure"]
impl crate::Writable for Scu44cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU44C to value 0"]
impl crate::Resettable for Scu44cSpec {}
