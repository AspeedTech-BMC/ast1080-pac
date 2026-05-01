#[doc = "Register `GPIO810` reader"]
pub type R = crate::R<Gpio810Spec>;
#[doc = "Register `GPIO810` writer"]
pub type W = crate::W<Gpio810Spec>;
#[doc = "Field `GPIO000WrPrivilegeOfMaster` reader - GPIO000 Write Privilege of Master"]
pub type Gpio000wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO000WrPrivilegeOfMaster` writer - GPIO000 Write Privilege of Master"]
pub type Gpio000wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO001WrPrivilegeOfMaster` reader - GPIO001 Write Privilege of Master"]
pub type Gpio001wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO001WrPrivilegeOfMaster` writer - GPIO001 Write Privilege of Master"]
pub type Gpio001wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO002WrPrivilegeOfMaster` reader - GPIO002 Write Privilege of Master"]
pub type Gpio002wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO002WrPrivilegeOfMaster` writer - GPIO002 Write Privilege of Master"]
pub type Gpio002wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO003WrPrivilegeOfMaster` reader - GPIO003 Write Privilege of Master"]
pub type Gpio003wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO003WrPrivilegeOfMaster` writer - GPIO003 Write Privilege of Master"]
pub type Gpio003wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO000 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio000wr_privilege_of_master(&self) -> Gpio000wrPrivilegeOfMasterR {
        Gpio000wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO001 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio001wr_privilege_of_master(&self) -> Gpio001wrPrivilegeOfMasterR {
        Gpio001wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO002 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio002wr_privilege_of_master(&self) -> Gpio002wrPrivilegeOfMasterR {
        Gpio002wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO003 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio003wr_privilege_of_master(&self) -> Gpio003wrPrivilegeOfMasterR {
        Gpio003wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO000 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio000wr_privilege_of_master(&mut self) -> Gpio000wrPrivilegeOfMasterW<Gpio810Spec> {
        Gpio000wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO001 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio001wr_privilege_of_master(&mut self) -> Gpio001wrPrivilegeOfMasterW<Gpio810Spec> {
        Gpio001wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO002 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio002wr_privilege_of_master(&mut self) -> Gpio002wrPrivilegeOfMasterW<Gpio810Spec> {
        Gpio002wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO003 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio003wr_privilege_of_master(&mut self) -> Gpio003wrPrivilegeOfMasterW<Gpio810Spec> {
        Gpio003wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio810::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio810::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio810Spec;
impl crate::RegisterSpec for Gpio810Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio810::R`](R) reader structure"]
impl crate::Readable for Gpio810Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio810::W`](W) writer structure"]
impl crate::Writable for Gpio810Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO810 to value 0xffff_ffff"]
impl crate::Resettable for Gpio810Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
