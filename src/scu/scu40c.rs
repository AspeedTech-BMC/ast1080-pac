#[doc = "Register `SCU40C` reader"]
pub type R = crate::R<Scu40cSpec>;
#[doc = "Register `SCU40C` writer"]
pub type W = crate::W<Scu40cSpec>;
#[doc = "Field `SCUMUXIO024` reader - SCU_MUX_IO024"]
pub type Scumuxio024R = crate::FieldReader;
#[doc = "Field `SCUMUXIO024` writer - SCU_MUX_IO024"]
pub type Scumuxio024W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO025` reader - SCU_MUX_IO025"]
pub type Scumuxio025R = crate::FieldReader;
#[doc = "Field `SCUMUXIO025` writer - SCU_MUX_IO025"]
pub type Scumuxio025W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO026` reader - SCU_MUX_IO026"]
pub type Scumuxio026R = crate::FieldReader;
#[doc = "Field `SCUMUXIO026` writer - SCU_MUX_IO026"]
pub type Scumuxio026W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO027` reader - SCU_MUX_IO027"]
pub type Scumuxio027R = crate::FieldReader;
#[doc = "Field `SCUMUXIO027` writer - SCU_MUX_IO027"]
pub type Scumuxio027W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO028` reader - SCU_MUX_IO028"]
pub type Scumuxio028R = crate::FieldReader;
#[doc = "Field `SCUMUXIO028` writer - SCU_MUX_IO028"]
pub type Scumuxio028W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO029` reader - SCU_MUX_IO029"]
pub type Scumuxio029R = crate::FieldReader;
#[doc = "Field `SCUMUXIO029` writer - SCU_MUX_IO029"]
pub type Scumuxio029W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO030` reader - SCU_MUX_IO030"]
pub type Scumuxio030R = crate::FieldReader;
#[doc = "Field `SCUMUXIO030` writer - SCU_MUX_IO030"]
pub type Scumuxio030W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO031` reader - SCU_MUX_IO031"]
pub type Scumuxio031R = crate::FieldReader;
#[doc = "Field `SCUMUXIO031` writer - SCU_MUX_IO031"]
pub type Scumuxio031W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO024"]
    #[inline(always)]
    pub fn scumuxio024(&self) -> Scumuxio024R {
        Scumuxio024R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO025"]
    #[inline(always)]
    pub fn scumuxio025(&self) -> Scumuxio025R {
        Scumuxio025R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO026"]
    #[inline(always)]
    pub fn scumuxio026(&self) -> Scumuxio026R {
        Scumuxio026R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO027"]
    #[inline(always)]
    pub fn scumuxio027(&self) -> Scumuxio027R {
        Scumuxio027R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO028"]
    #[inline(always)]
    pub fn scumuxio028(&self) -> Scumuxio028R {
        Scumuxio028R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO029"]
    #[inline(always)]
    pub fn scumuxio029(&self) -> Scumuxio029R {
        Scumuxio029R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO030"]
    #[inline(always)]
    pub fn scumuxio030(&self) -> Scumuxio030R {
        Scumuxio030R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO031"]
    #[inline(always)]
    pub fn scumuxio031(&self) -> Scumuxio031R {
        Scumuxio031R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO024"]
    #[inline(always)]
    pub fn scumuxio024(&mut self) -> Scumuxio024W<Scu40cSpec> {
        Scumuxio024W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO025"]
    #[inline(always)]
    pub fn scumuxio025(&mut self) -> Scumuxio025W<Scu40cSpec> {
        Scumuxio025W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO026"]
    #[inline(always)]
    pub fn scumuxio026(&mut self) -> Scumuxio026W<Scu40cSpec> {
        Scumuxio026W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO027"]
    #[inline(always)]
    pub fn scumuxio027(&mut self) -> Scumuxio027W<Scu40cSpec> {
        Scumuxio027W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO028"]
    #[inline(always)]
    pub fn scumuxio028(&mut self) -> Scumuxio028W<Scu40cSpec> {
        Scumuxio028W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO029"]
    #[inline(always)]
    pub fn scumuxio029(&mut self) -> Scumuxio029W<Scu40cSpec> {
        Scumuxio029W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO030"]
    #[inline(always)]
    pub fn scumuxio030(&mut self) -> Scumuxio030W<Scu40cSpec> {
        Scumuxio030W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO031"]
    #[inline(always)]
    pub fn scumuxio031(&mut self) -> Scumuxio031W<Scu40cSpec> {
        Scumuxio031W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu40c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu40c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu40cSpec;
impl crate::RegisterSpec for Scu40cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu40c::R`](R) reader structure"]
impl crate::Readable for Scu40cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu40c::W`](W) writer structure"]
impl crate::Writable for Scu40cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU40C to value 0"]
impl crate::Resettable for Scu40cSpec {}
