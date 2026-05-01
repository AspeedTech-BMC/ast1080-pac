#[doc = "Register `GPIO88C` reader"]
pub type R = crate::R<Gpio88cSpec>;
#[doc = "Register `GPIO88C` writer"]
pub type W = crate::W<Gpio88cSpec>;
#[doc = "Field `GPIO124WrPrivilegeOfMaster` reader - GPIO124 Write Privilege of Master"]
pub type Gpio124wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO124WrPrivilegeOfMaster` writer - GPIO124 Write Privilege of Master"]
pub type Gpio124wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO125WrPrivilegeOfMaster` reader - GPIO125 Write Privilege of Master"]
pub type Gpio125wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO125WrPrivilegeOfMaster` writer - GPIO125 Write Privilege of Master"]
pub type Gpio125wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO126WrPrivilegeOfMaster` reader - GPIO126 Write Privilege of Master"]
pub type Gpio126wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO126WrPrivilegeOfMaster` writer - GPIO126 Write Privilege of Master"]
pub type Gpio126wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO127WrPrivilegeOfMaster` reader - GPIO127 Write Privilege of Master"]
pub type Gpio127wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO127WrPrivilegeOfMaster` writer - GPIO127 Write Privilege of Master"]
pub type Gpio127wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO124 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio124wr_privilege_of_master(&self) -> Gpio124wrPrivilegeOfMasterR {
        Gpio124wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO125 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio125wr_privilege_of_master(&self) -> Gpio125wrPrivilegeOfMasterR {
        Gpio125wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO126 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio126wr_privilege_of_master(&self) -> Gpio126wrPrivilegeOfMasterR {
        Gpio126wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO127 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio127wr_privilege_of_master(&self) -> Gpio127wrPrivilegeOfMasterR {
        Gpio127wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO124 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio124wr_privilege_of_master(&mut self) -> Gpio124wrPrivilegeOfMasterW<Gpio88cSpec> {
        Gpio124wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO125 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio125wr_privilege_of_master(&mut self) -> Gpio125wrPrivilegeOfMasterW<Gpio88cSpec> {
        Gpio125wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO126 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio126wr_privilege_of_master(&mut self) -> Gpio126wrPrivilegeOfMasterW<Gpio88cSpec> {
        Gpio126wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO127 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio127wr_privilege_of_master(&mut self) -> Gpio127wrPrivilegeOfMasterW<Gpio88cSpec> {
        Gpio127wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#31\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio88c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio88c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio88cSpec;
impl crate::RegisterSpec for Gpio88cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio88c::R`](R) reader structure"]
impl crate::Readable for Gpio88cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio88c::W`](W) writer structure"]
impl crate::Writable for Gpio88cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO88C to value 0xffff_ffff"]
impl crate::Resettable for Gpio88cSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
